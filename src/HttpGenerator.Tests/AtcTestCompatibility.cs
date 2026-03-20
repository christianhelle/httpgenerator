using AutoFixture;
using AutoFixture.AutoNSubstitute;
using AutoFixture.Xunit2;
using Xunit;
using Xunit.Sdk;

namespace Atc.Test;

[AttributeUsage(AttributeTargets.Method, AllowMultiple = false, Inherited = true)]
public sealed class AutoNSubstituteDataAttribute : AutoDataAttribute
{
    public AutoNSubstituteDataAttribute()
        : base(FixtureFactory.Create)
    {
    }
}

[AttributeUsage(AttributeTargets.Method, AllowMultiple = true, Inherited = true)]
public sealed class InlineAutoNSubstituteDataAttribute : CompositeDataAttribute
{
    public InlineAutoNSubstituteDataAttribute(params object[] data)
        : base(
        new DataAttribute[]
        {
            new InlineDataAttribute(data),
            new AutoNSubstituteDataAttribute(),
        })
    {
    }
}

public static class FixtureFactory
{
    public static IFixture Create()
    {
        var fixture = new Fixture();

        foreach (var behavior in fixture.Behaviors.OfType<ThrowingRecursionBehavior>().ToList())
        {
            fixture.Behaviors.Remove(behavior);
        }

        fixture.Behaviors.Add(new OmitOnRecursionBehavior());
        fixture.Customize(new AutoNSubstituteCustomization
        {
            ConfigureMembers = false,
            GenerateDelegates = true,
        });

        return fixture;
    }
}
